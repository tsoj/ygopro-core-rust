//! One or Eight
//! 一か八か
use super::prelude::*;

pub const ID: u32 = 74202664;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetLP(tp)<Duel.GetLP(1-tp)
    duel::get_lp(d, a.tp) < duel::get_lp(d, 1 - a.tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0;
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)==0 then return end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) == 0 {
        return;
    }
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, tp, 1);
    // local tc=Duel.GetDecktopGroup(tp,1):GetFirst()
    let tc = group::get_first(d, &duel::get_decktop_group(d, tp, 1)).nil();
    // if tc:IsMonster() and (tc:IsLevel(1) or tc:IsLevel(8)) then
    if card::is_monster(d, tc) && (card::is_level(d, tc, &[1]) || card::is_level(d, tc, &[8])) {
        // aux.ToHandOrElse(tc,tp,
        //     function(c) return tc:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end,
        //     function(c) Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end,
        //     aux.Stringid(id,1))
        let e = a.e;
        aux::to_hand_or_else(
            d,
            tc,
            tp,
            Some(&|d, _c| card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0),
            Some(&|d, _cs| {
                duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                0
            }),
            Some(aux::stringid(ID, 1)),
        );
    } else {
        // if not Duel.SelectEffectYesNo(1-tp,e:GetHandler()) then return end
        let h = effect::get_handler(d, a.e).nil();
        if !duel::select_effect_yes_no(d, 1 - tp, h, None) {
            return;
        }
        // local op=Duel.SelectOption(1-tp,aux.Stringid(id,2),aux.Stringid(id,3))
        let op = duel::select_option(d, 1 - tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]);
        // local p=e:GetHandler():GetControler()
        let p = card::get_controler(d, h);
        // if op==0 then
        if op == 0 {
            // Duel.SetLP(p,1000)
            duel::set_lp(d, p, 1000);
        // elseif op==1 then
        } else if op == 1 {
            // local recv=8000-Duel.GetLP(1-p)
            let mut recv = 8000 - duel::get_lp(d, 1 - p);
            // if recv<0 then recv=0 end
            if recv < 0 {
                recv = 0;
            }
            // Duel.Recover(1-p,recv,REASON_EFFECT)
            duel::recover(d, 1 - p, recv as i64, REASON_EFFECT, false, None);
        }
    }
}
