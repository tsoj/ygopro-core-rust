//! Charmers of the Grand Circle
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 48016074;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CHARMER, SET_FAMILIAR_POSSESSED, SET_POSSESSED],
    material_setcode: &[SET_CHARMER as u32, SET_FAMILIAR_POSSESSED as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 2+ "Charmer" and/or "Familiar-Possessed" monsters
    // Fusion.AddProcMixRep(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,{SET_CHARMER,SET_FAMILIAR_POSSESSED}),2,99)
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| {
            card::is_set_card_ctx(d, c, &[SET_CHARMER as u16, SET_FAMILIAR_POSSESSED as u16], Some(fc), st, Some(tp))
        })),
        2,
        99,
        vec![],
    );
    // Keep track of the number of different original Attributes of the monsters used as its material
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(function(e,c) e:SetLabel(c:GetMaterial():GetClassCount(Card.GetOriginalAttribute)) end)
    effect::set_value_fn(d, e0, effect_card_value(material_check));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Repeat the process of applying 1 of these effects, up to the number of different original Attributes of the monsters used as its material (max. 4)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_fusion_summoned(d, h)
    })));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetLabelObject(e0)
    effect::set_label_object(d, e1, Value::Effect(e0));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e,c) e:SetLabel(c:GetMaterial():GetClassCount(Card.GetOriginalAttribute)) end` (the value of `e0`)
fn material_check(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    let mat = card::get_material(d, c);
    let n = group::get_class_count(d, &mat, &|d, x| card::get_original_attribute(d, x) as i64);
    effect::set_label(d, e, &[n as i64]);
    // the function returns nothing: the core reads nil as 0
    0
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_POSSESSED) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_POSSESSED as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_SPELLCASTER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local material_check_eff=e:GetLabelObject()
    let material_check_eff = effect::get_label_object(d, a.e).as_effect().nil();
    // local label=material_check_eff:GetLabel()
    let label = effect::get_label(d, material_check_eff);
    // if chk==0 then return label>0 end
    if a.chk == Some(0) {
        return label > 0;
    }
    // material_check_eff:SetLabel(0)
    effect::set_label(d, material_check_eff, &[0]);
    // Duel.SetTargetParam(label)
    duel::set_target_param(d, label as u32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_ATKCHANGE,e:GetHandler(),1,tp,800)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(h)), 1, a.tp, 800);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_ONFIELD) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local max_count=math.min(Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM),4)
    let max_count = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int().min(4);
    // local remaining_count=max_count
    let mut remaining_count = max_count;
    // repeat
    loop {
        // b1=c:IsRelateToEffect(e) and c:IsFaceup()
        let b1 = card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c);
        // b2=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
        let b2 = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
        // b3=Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
        let b3 = duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_hand(d, x, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
        // b4=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,nil,e,tp)
        let b4 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp))), tp, LOCATION_GRAVE, 0, 1, NO_EX);
        // if not (b1 or b2 or b3 or b4) then return end
        if !(b1 || b2 || b3 || b4) {
            return;
        }
        // b5=remaining_count<max_count
        let b5 = remaining_count < max_count;
        // op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)},{b3,aux.Stringid(id,3)},{b4,aux.Stringid(id,4)},{b5,aux.Stringid(id,5)})
        let op = duel::select_effect(
            d,
            tp,
            &[
                (b1, aux::stringid(ID, 1)),
                (b2, aux::stringid(ID, 2)),
                (b3, aux::stringid(ID, 3)),
                (b4, aux::stringid(ID, 4)),
                (b5, aux::stringid(ID, 5)),
            ],
        )
        .nil();
        // remaining_count=remaining_count-1
        remaining_count -= 1;
        if op == 1 {
            // This card gains 800 ATK
            // c:UpdateAttack(800)
            card::update_attack(d, c, 800, None, None, None);
        } else if op == 2 {
            // Add 1 "Possessed" Spell/Trap from your Deck to your hand
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
            let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SendtoHand(g,nil,REASON_EFFECT)
                duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,g)
                duel::confirm_cards(d, 1 - tp, g);
            }
        } else if op == 3 {
            // Return 1 card on the field to the hand
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
            let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_able_to_hand(d, x, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.HintSelection(g)
                duel::hint_selection(d, g.clone(), true);
                // Duel.SendtoHand(g,nil,REASON_EFFECT)
                duel::send_to_hand(d, g, None, REASON_EFFECT, None);
            }
        } else if op == 4 {
            // Special Summon 1 Spellcaster monster from your GY
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
            let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        } else if op == 5 {
            // Finish applying effects
            // return
            return;
        }
        // until remaining_count==0
        if remaining_count == 0 {
            break;
        }
    }
}
