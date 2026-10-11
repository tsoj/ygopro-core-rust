//! SPYRAL Tough
use super::prelude::*;

pub const ID: u32 = 20584712;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[41091257], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card's name becomes "SPYRAL Super Agent" while it is on the field or in the GY.
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetValue(41091257)
    effect::set_value(d, e1, 41091257);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Reveal the top card of your opponent's Deck, and if you do, destroy a targeted card if the revealed card is the declared type
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil)
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX) && duel::get_field_group_count(d, tp, 0, LOCATION_DECK) > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CARDTYPE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CARDTYPE as u64);
    // e:SetLabel(Duel.SelectOption(tp,DECLTYPE_MONSTER,DECLTYPE_SPELL,DECLTYPE_TRAP))
    let opt = duel::select_option(d, tp, true, &[DECLTYPE_MONSTER as u64, DECLTYPE_SPELL as u64, DECLTYPE_TRAP as u64]);
    effect::set_label(d, a.e, &[opt as i64]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
    let tc = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, tc, 1, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)==0 then return end
    if duel::get_field_group_count(d, tp, 0, LOCATION_DECK) == 0 {
        return;
    }
    // local dc=Duel.GetFirstTarget()
    let dc = duel::first_target(d).nil();
    // if not dc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, dc, a.e) {
        return;
    }
    // Duel.ConfirmDecktop(1-tp,1)
    duel::confirm_decktop(d, 1 - tp, 1);
    // local g=Duel.GetDecktopGroup(1-tp,1)
    let g = duel::get_decktop_group(d, 1 - tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // local opt=e:GetLabel()
    let opt = effect::get_label(d, a.e);
    // if (opt==0 and tc:IsMonster()) or (opt==1 and tc:IsSpell()) or (opt==2 and tc:IsTrap()) then
    if (opt == 0 && card::is_monster(d, tc.nil()))
        || (opt == 1 && card::is_spell(d, tc.nil()))
        || (opt == 2 && card::is_trap(d, tc.nil()))
    {
        // Duel.Destroy(dc,REASON_EFFECT)
        duel::destroy(d, dc, REASON_EFFECT, None, None);
    }
}
