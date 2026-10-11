//! Tails of the Fairy Tails
//! テールズオブ妖精伝姫
use super::prelude::*;

pub const ID: u32 = 82119326;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_FAIRY_TAIL],
    listed_names: &[CARD_PRINCE_OF_FAIRIES],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // The equipped monster gains 1000 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(1000)
    effect::set_value(d, e1, 1000);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Fusion Summon 1 Spellcaster Fusion Monster from your Extra Deck, using monsters from your hand or field, when you do, you can also use any "Fairy Prince" your opponent controls
    // local fusion_params={
    //     handler=c,
    //     fusfilter=aux.FilterBoolFunction(Card.IsRace,RACE_SPELLCASTER),
    //     extrafil=function(e,tp,mg) return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(aux.FaceupFilter(Card.IsCode,CARD_PRINCE_OF_FAIRIES)),tp,0,LOCATION_ONFIELD,nil) end
    // }
    // (`handler` is only used by Fusion.CreateSummonEff, not by SummonEffTG/SummonEffOP)
    let fusion_params = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, cc, _tp| card::is_race(d, cc, RACE_SPELLCASTER))),
        extrafil: Some(std::sync::Arc::new(|d, _e, tp, _mg| {
            let g = duel::get_matching_group(
                d,
                Some(&|d, cc| card::is_monster(d, cc) && card::is_faceup(d, cc) && card::is_code(d, cc, &[CARD_PRINCE_OF_FAIRIES])),
                tp,
                0,
                LOCATION_ONFIELD,
                NO_EX,
            );
            (Some(g), None)
        })),
        ..Default::default()
    };
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON | CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(Fusion.SummonEffTG(fusion_params))
    let tg = fusion::summon_eff_tg(d, fusion_params.clone());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(fusion_params))
    let op = fusion::summon_eff_op(d, fusion_params);
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Equip this card to a "Fairy Tail" monster on the field, then immediately after this effect resolves, you can Normal Summon 1 Spellcaster monster with 1850 ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_EQUIP+CATEGORY_SUMMON)
    effect::set_category(d, e3, CATEGORY_EQUIP | CATEGORY_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.eqtg)
    effect::set_target(d, e3, Some(cost_target(eqtg)));
    // e3:SetOperation(s.eqop)
    effect::set_operation(d, e3, Some(operation(eqop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_FAIRY_TAIL)`
fn fairy_tail_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_FAIRY_TAIL as u16])
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_FAIRY_TAIL),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&fairy_tail_faceup), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, tp, 0);
    true
}
/// `s.nsfilter(c)`
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsAttack(1850) and c:IsSummonable(true,nil)
    card::is_race(d, c, RACE_SPELLCASTER) && card::is_attack(d, c, &[1850]) && card::is_summonable(d, c, true, None, 0, None)
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // local sc=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsSetCard,SET_FAIRY_TAIL),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&fairy_tail_faceup), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let sc = match group::get_first(d, &sel) {
            Some(sc) => sc,
            // if not sc then return end
            None => return,
        };
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // if Duel.Equip(tp,c,sc) and Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::equip(d, tp, c, sc, true, false)
            && duel::is_existing_matching_card(d, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
            // local nsc=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil):GetFirst()
            let sel2 = duel::select_matching_card(d, tp, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            // if nsc then
            if let Some(nsc) = group::get_first(d, &sel2) {
                // Duel.Summon(tp,nsc,true,nil)
                duel::summon(d, tp, nsc, true, None, 0, None);
            }
        }
    }
}
