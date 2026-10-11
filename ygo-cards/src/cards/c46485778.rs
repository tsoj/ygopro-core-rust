//! ポワソニエル・ド・ヌーベルズ (Poissonniere de Nouvelles)
use super::prelude::*;

pub const ID: u32 = 46485778;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RECIPE, SET_NOUVELLES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Attributes
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Special Summon itself from the Pendulum Zone and change the position of a monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place itself in the Pendulum Zone if it is Tributed
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2, EVENT_RELEASE);
    // e2:SetCondition(s.pencon)
    effect::set_condition(d, e2, Some(condition(pencon)));
    // e2:SetTarget(s.pentg)
    effect::set_target(d, e2, Some(cost_target(pentg)));
    // e2:SetOperation(s.penop)
    effect::set_operation(d, e2, Some(operation(penop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Activate 1 of these effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.efftg)
    effect::set_target(d, e3, Some(cost_target(efftg)));
    // e3:SetOperation(s.effop)
    effect::set_operation(d, e3, Some(operation(effop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRitualMonster),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&|d: &mut Duel, c: CardId| card::is_faceup(d, c) && card::is_ritual_monster(d, c)),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // (s.sptg has no chkc parameter: the chkc branch of the Lua reads a nil global and is skipped)
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingTarget(Card.IsCanChangePosition,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    // end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_target(
                d,
                Some(&|d: &mut Duel, x: CardId| card::is_can_change_position(d, x)),
                a.tp,
                LOCATION_MZONE,
                LOCATION_MZONE,
                1,
                NO_EX,
            );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsCanChangePosition,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(
        d,
        a.tp,
        Some(&|d: &mut Duel, x: CardId| card::is_can_change_position(d, x)),
        a.tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.ChangePosition(tc,POS_FACEUP_DEFENSE,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)
        duel::change_position(
            d,
            tc.nil(),
            POS_FACEUP_DEFENSE as u8,
            Some(POS_FACEUP_ATTACK as u8),
            Some(POS_FACEUP_ATTACK as u8),
            Some(POS_FACEUP_ATTACK as u8),
            false,
        );
    }
}
// function s.pencon(e,tp,eg,ep,ev,re,r,rp)
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFaceup() and c:IsLocation(LOCATION_EXTRA) and c:IsPreviousLocation(LOCATION_ONFIELD)
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_EXTRA) && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
// function s.pentg(e,tp,eg,ep,ev,re,r,rp,chk)
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    true
}
// function s.penop(e,tp,eg,ep,ev,re,r,rp)
fn penop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.CheckPendulumZones(tp) then return end
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_RECIPE) or (c:IsRitualMonster() and c:IsLevel(1))) and c:IsAbleToHand()
    (card::is_set_card(d, c, &[SET_RECIPE as u16]) || (card::is_ritual_monster(d, c) && card::is_level(d, c, &[1])))
        && card::is_able_to_hand(d, c, None)
}
// function s.rmvfilter(c)
fn rmvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RECIPE) and c:IsAbleToRemove()
    card::is_set_card(d, c, &[SET_RECIPE as u16]) && card::is_able_to_remove(d, c, None, None, None)
}
// function s.nvlfilter(c,e,tp,lv)
fn nvlfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:IsSetCard(SET_NOUVELLES) and c:IsRitualMonster()
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,true) and c:IsLevelBelow(lv)
    card::is_set_card(d, c, &[SET_NOUVELLES as u16])
        && card::is_ritual_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, true, None, None, None)
        && card::is_level_below(d, c, lv)
}
// function s.spfilter(c,e,tp,lv)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:IsSetCard(SET_NOUVELLES) and c:IsRitualMonster()
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,true) and c:IsLevel(lv)
    card::is_set_card(d, c, &[SET_NOUVELLES as u16])
        && card::is_ritual_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, true, None, None, None)
        && card::is_level(d, c, &[lv])
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local rg=Duel.GetMatchingGroup(s.rmvfilter,tp,LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&rmvfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local b1=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #rg>0
    //     and Duel.IsExistingMatchingCard(s.nvlfilter,tp,LOCATION_HAND,0,1,nil,e,tp,#rg)
    let b2 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && rg.len() > 0
        && duel::is_existing_matching_card(
            d,
            Some(&|d: &mut Duel, c: CardId| nvlfilter(d, c, a.e, a.tp, rg.len() as u32)),
            a.tp,
            LOCATION_HAND,
            0,
            1,
            NO_EX,
        );
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,3)}, --Search 1 Level 1 Ritual Monster or 1 "Recipe" card
    //     {b2,aux.Stringid(id,4)}) --Special Summon 1 "Nouvelles" Ritual Monster from your hand
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    }
    true
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp,#sg)
    let lv = sg.len() as u32;
    let e = e.nil();
    let res = duel::is_existing_matching_card(
        d,
        Some(&|d: &mut Duel, c: CardId| spfilter(d, c, e, tp, lv)),
        tp,
        LOCATION_HAND,
        0,
        1,
        NO_EX,
    );
    (res, false)
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if g.len() > 0 {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    } else if op == 2 {
        // local rg=Duel.GetMatchingGroup(s.rmvfilter,tp,LOCATION_GRAVE,0,nil)
        let rg = duel::get_matching_group(d, Some(&rmvfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // if #rg==0 then return end
        if rg.len() == 0 {
            return;
        }
        // local rmg=aux.SelectUnselectGroup(rg,e,tp,1,#rg,s.rescon,1,tp,HINTMSG_REMOVE,s.rescon)
        let rmg = aux::select_unselect_group(
            d,
            &rg,
            Some(a.e),
            a.tp,
            Some(1),
            Some(rg.len()),
            Some(&rescon),
            1,
            a.tp,
            Some(HINTMSG_REMOVE as u64),
            Some(&rescon),
            None,
            false,
        )
        .unwrap_or_default();
        // if #rmg>0 and Duel.Remove(rmg,nil,REASON_EFFECT)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
        if rmg.len() > 0
            && duel::remove(d, rmg.clone(), 0, REASON_EFFECT, None, None) > 0
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp,#rmg)
            let rmg_len = rmg.len() as u32;
            let g = duel::select_matching_card(
                d,
                a.tp,
                Some(&|d: &mut Duel, c: CardId| spfilter(d, c, a.e, a.tp, rmg_len)),
                a.tp,
                LOCATION_HAND,
                0,
                1,
                1,
                false,
                NO_EX,
            )
            .nil();
            // if #g>0 then
            if g.len() > 0 {
                // Duel.SpecialSummon(g,0,tp,tp,false,true,POS_FACEUP)
                duel::special_summon(d, g, 0, a.tp, a.tp, false, true, POS_FACEUP as u8, None);
            }
        }
    }
}
