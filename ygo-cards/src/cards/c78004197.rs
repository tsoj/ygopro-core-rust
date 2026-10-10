//! Goldd, Wu-Lord of Dark World
use super::prelude::*;

pub const ID: u32 = 78004197;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(e:GetHandler():GetPreviousControler())
    let c = effect::get_handler(d, a.e).nil();
    let prev = card::get_previous_controler(d, c) as i64;
    effect::set_label(d, a.e, &[prev]);
    // return e:GetHandler():IsPreviousLocation(LOCATION_HAND) and r&(REASON_DISCARD|REASON_EFFECT)==REASON_DISCARD|REASON_EFFECT
    card::is_previous_location(d, c, LOCATION_HAND) && (a.r & (REASON_DISCARD | REASON_EFFECT)) == (REASON_DISCARD | REASON_EFFECT)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    // if rp~=tp and tp==e:GetLabel() and Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_ONFIELD,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if a.rp != a.tp
        && a.tp as i64 == effect::get_label(d, a.e)
        && duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,1,2,nil)
        let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, 2, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
        let n = g.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
    }
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if e:GetHandler():IsRelateToEffect(e) and Duel.SpecialSummon(e:GetHandler(),0,tp,tp,false,false,POS_FACEUP)>0 and e:GetLabel()==1 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && effect::get_label(d, a.e) == 1
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
        let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
        // if not g then return end
        let Some(g) = g else {
            return;
        };
        // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
        let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
