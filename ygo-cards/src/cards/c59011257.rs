//! Fallin' Cheatah
use super::prelude::*;

pub const ID: u32 = 59011257;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1a:SetCode(EFFECT_UNRELEASABLE_SUM)
    effect::set_code(d, e1a, EFFECT_UNRELEASABLE_SUM);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetValue(1)
    effect::set_value(d, e1a, 1);
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_UNRELEASABLE_NONSUM)
    effect::set_code(d, e1b, EFFECT_UNRELEASABLE_NONSUM);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e1c=e1a:Clone()
    let e1c = effect::clone(d, e1a);
    // e1c:SetCode(EFFECT_CANNOT_BE_MATERIAL)
    effect::set_code(d, e1c, EFFECT_CANNOT_BE_MATERIAL);
    // e1c:SetValue(aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK))
    effect::set_value_fn(d, e1c, cannot_mat_filter());
    // c:RegisterEffect(e1c)
    card::register_effect(d, c, e1c, false);
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2a, aux::stringid(ID, 0));
    // e2a:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e2a, CATEGORY_CONTROL);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2a:SetProperty(EFFECT_FLAG_CARD_TARGET,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2a, EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2a:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2a, EVENT_CUSTOM + ID);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetTarget(s.ctrltg)
    effect::set_target(d, e2a, Some(cost_target(ctrltg)));
    // e2a:SetOperation(s.ctrlop)
    effect::set_operation(d, e2a, Some(operation(ctrlop)));
    // e2a:SetLabelObject(Group.CreateGroup())
    effect::set_label_object(d, e2a, Value::Group(CardSet::new()));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // e2b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2b, LOCATION_MZONE);
    // e2b:SetLabelObject(e2a)
    effect::set_label_object(d, e2b, Value::Effect(e2a));
    // e2b:SetOperation(s.regop)
    effect::set_operation(d, e2b, Some(operation(regop)));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK)`
fn cannot_mat_filter() -> LuaFn {
    aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK])
}
/// The group kept in the label object of an effect (`e:GetLabelObject()` as a group).
fn label_group(d: &mut Duel, e: EffectId) -> CardSet {
    match effect::get_label_object(d, e) {
        Value::Group(g) => g,
        _ => crate::lua_error("attempt to index a nil value"),
    }
}
/// `s.tgfilter(c,e,opp)`
fn tgfilter(d: &mut Duel, c: CardId, e: Option<EffectId>, opp: u8) -> bool {
    // return c:IsControler(opp) and c:IsLocation(LOCATION_MZONE) and (not e or c:IsCanBeEffectTarget(e))
    card::is_controler(d, c, opp) && card::is_location(d, c, LOCATION_MZONE) && (e.is_none() || card::is_can_be_effect_target(d, c, e))
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local tg=eg:Filter(s.tgfilter,nil,nil,1-tp)
    let eg = a.eg();
    let tg = group::filter(d, &eg, &|d, c| tgfilter(d, c, None, 1 - a.tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e2a = match effect::get_label_object(d, a.e) {
            Value::Effect(x) => x,
            _ => crate::lua_error("attempt to index a nil value"),
        };
        let mut g = label_group(d, e2a);
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e2a, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        let h = effect::get_handler(d, a.e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
    }
}
/// `s.ctrlfilter(c,tp)`
fn ctrlfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE,tp,LOCATION_REASON_CONTROL)>0
    let p = card::get_controler(d, c);
    duel::get_location_count(d, p, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
}
/// `s.ctrltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn ctrltg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject():Filter(s.tgfilter,nil,e,1-tp):Match(s.ctrlfilter,nil,tp)
    let lo = label_group(d, a.e);
    let mut g = group::filter(d, &lo, &|d, c| tgfilter(d, c, Some(a.e), 1 - a.tp), NO_EX);
    group::match_(d, &mut g, &|d, c| ctrlfilter(d, c, a.tp), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.tgfilter(chkc,e,1-tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && tgfilter(d, chkc, Some(a.e), 1 - a.tp);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if #g>0 then
    if !g.is_empty() {
        // local tc=nil
        // if #g>1 then
        let tc = if g.len() > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // tc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            group::get_first(d, &sel)
        } else {
            // tc=g:GetFirst()
            group::get_first(d, &g)
        };
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc.nil());
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(c)), 1, a.tp, 0);
    }
    true
}
/// `s.ctrlop(e,tp,eg,ep,ev,re,r,rp)`
fn ctrlop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.GetControl(c,tc:GetControler()) and c:IsFaceup() then
    let Some(tc) = tc else { return };
    if card::is_relate_to_effect(d, tc, a.e) {
        let p = card::get_controler(d, tc);
        duel::get_control(d, CardSet::from_card(c), p, 0, 0, None, None);
        if card::is_faceup(d, c) {
            // c:SetCardTarget(tc)
            card::set_card_target(d, c, tc);
            // local fid=c:GetFieldID()
            let fid = card::get_field_id(d, c);
            // tc:RegisterFlagEffect(id+1,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET),EFFECT_FLAG_CLIENT_HINT,1,fid,aux.Stringid(id,1))
            card::register_flag_effect(d, tc, ID + 1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), EFFECT_FLAG_CLIENT_HINT, 1, fid as i64, aux::stringid(ID, 1));
            // local e1a=Effect.CreateEffect(c)
            let e1a = effect::create_effect(d, c);
            // e1a:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
            // e1a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1a, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1a:SetCode(EFFECT_UNRELEASABLE_SUM)
            effect::set_code(d, e1a, EFFECT_UNRELEASABLE_SUM);
            // e1a:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1a, LOCATION_MZONE);
            // e1a:SetTarget(function(e,c) return c:GetFlagEffectLabel(id+1)==fid end)
            effect::set_target(d, e1a, Some(effect_card_fn(move |d, _e, x| card::get_flag_effect_label(d, x, ID + 1).first() == Some(&(fid as i64)))));
            // e1a:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
            effect::set_target_range(d, e1a, LOCATION_MZONE, LOCATION_MZONE);
            // e1a:SetValue(1)
            effect::set_value(d, e1a, 1);
            // e1a:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1a, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e1a)
            card::register_effect(d, c, e1a, false);
            // local e1b=e1a:Clone()
            let e1b = effect::clone(d, e1a);
            // e1b:SetCode(EFFECT_UNRELEASABLE_NONSUM)
            effect::set_code(d, e1b, EFFECT_UNRELEASABLE_NONSUM);
            // c:RegisterEffect(e1b)
            card::register_effect(d, c, e1b, false);
            // local e1c=e1a:Clone()
            let e1c = effect::clone(d, e1a);
            // e1c:SetCode(EFFECT_CANNOT_BE_MATERIAL)
            effect::set_code(d, e1c, EFFECT_CANNOT_BE_MATERIAL);
            // e1c:SetValue(aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK))
            effect::set_value_fn(d, e1c, cannot_mat_filter());
            // c:RegisterEffect(e1c)
            card::register_effect(d, c, e1c, false);
        }
    }
}
