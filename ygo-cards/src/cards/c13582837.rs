//! Infernity Randomizer
use super::prelude::*;

pub const ID: u32 = 13582837;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0
    duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) == 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,1,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 1, a.tp, 1);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)~=0 then return end
    if duel::get_field_group_count(d, tp, LOCATION_HAND, 0) != 0 {
        return;
    }
    // local g=Duel.GetDecktopGroup(tp,1)
    let g = duel::get_decktop_group(d, tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, tp, 1, REASON_EFFECT);
    // if tc then
    if let Some(tc) = tc {
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - tp, tc);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if tc:IsMonster() then
        if card::is_monster(d, tc) {
            // Duel.Damage(1-tp,tc:GetLevel()*200,REASON_EFFECT)
            let amount = card::get_level(d, tc) as i64 * 200;
            duel::damage(d, 1 - tp, amount, REASON_EFFECT, false, None);
        } else {
            // Duel.Damage(tp,500,REASON_EFFECT)
            duel::damage(d, tp, 500, REASON_EFFECT, false, None);
        }
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
    }
}
