//! Caius the Mega Monarch
use super::prelude::*;

pub const ID: u32 = 87288189;

/// `local LOCATION_HAND_DECK_EXTRA_GRAVE=LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA|LOCATION_GRAVE`
const LOCATION_HAND_DECK_EXTRA_GRAVE: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 1 tribute
    // local e1=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e1 = aux::add_normal_summon_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE),
        Some(aux::stringid(ID, 0)),
        Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _tp: u8| otfilter(d, c))),
        None,
    );
    // local e2=aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e2 = aux::add_normal_set_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE),
        Some(aux::stringid(ID, 0)),
        Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _tp: u8| otfilter(d, c))),
        None,
    );
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    // e4:SetValue(s.valcheck)
    effect::set_value_fn(d, e4, lua_fn(valcheck));
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.otfilter(c)`
fn otfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:GetSummonType()&SUMMON_TYPE_TRIBUTE)==SUMMON_TYPE_TRIBUTE
    (card::get_summon_type(d, c) & SUMMON_TYPE_TRIBUTE) == SUMMON_TYPE_TRIBUTE
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_TRIBUTE
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_TRIBUTE
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ct=1
    let mut ct = 1;
    // if e:GetLabel()==1 then e:SetLabel(0) ct=2 end
    if effect::get_label(d, a.e) == 1 {
        effect::set_label(d, a.e, &[0]);
        ct = 2;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,ct,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, ct, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), g.len() as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 1000);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,g:GetFirst():GetControler(),LOCATION_HAND_DECK_EXTRA_GRAVE)
    let first = group::get_first(d, &g).nil();
    let p = card::get_controler(d, first);
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, p, LOCATION_HAND_DECK_EXTRA_GRAVE as i32);
    true
}
/// `s.rmfilter(c,code)`
fn rmfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and (c:IsLocation(LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA) or aux.SpElimFilter(c,true))
    card::is_code(d, c, &[code])
        && (card::is_location(d, c, LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA) || aux::sp_elim_filter(d, c, true, false))
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 and Duel.Remove(g,POS_FACEUP,REASON_EFFECT)~=0 then
    if !g.is_empty() && duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 {
        // Duel.Damage(1-tp,1000,REASON_EFFECT)
        duel::damage(d, 1 - tp, 1000, REASON_EFFECT, false, None);
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // local rg=Group.CreateGroup()
        let mut rg = CardSet::new();
        // local tc=og:GetFirst()
        let _ = group::get_first(d, &og);
        // for tc in aux.Next(og) do
        for tc in group::iter(d, &og) {
            // if tc:IsAttribute(ATTRIBUTE_DARK) then
            if card::is_attribute(d, tc, ATTRIBUTE_DARK) {
                // local sg=Duel.GetMatchingGroup(s.rmfilter,tc:GetControler(),LOCATION_HAND_DECK_EXTRA_GRAVE,0,nil,tc:GetCode())
                let controler = card::get_controler(d, tc);
                let code = card::get_code(d, tc).first().copied().unwrap_or(0);
                let sg = duel::get_matching_group(d, Some(&|d, c| rmfilter(d, c, code)), controler, LOCATION_HAND_DECK_EXTRA_GRAVE, 0, NO_EX);
                // rg:Merge(sg)
                group::add_card(&mut rg, &sg);
            }
        }
        // if #rg>0 then
        if !rg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) {
    // local g=c:GetMaterial()
    let c = a.c(1);
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_DARK) then
    let e3 = effect::get_label_object(d, a.e(0)).as_effect().nil();
    if group::is_exists(d, &g, &|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK), 1, NO_EX) {
        // e:GetLabelObject():SetLabel(1)
        effect::set_label(d, e3, &[1]);
    } else {
        // e:GetLabelObject():SetLabel(0)
        effect::set_label(d, e3, &[0]);
    }
}
