//! Beargram, Shelled Emperor of the Forest Crown
use super::prelude::*;

pub const ID: u32 = 27134209;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableUnsummonable()
    card::enable_unsummonable(d, c);
    // Can only be Special Summoned once per turn
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Must be Special Summoned by its own procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.FALSE)
    effect::set_value_fn(d, e1, lua_fn(aux_false));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon procedure
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_HAND | LOCATION_GRAVE);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Prevent response to the activation of your Spell/Traps
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetOperation(s.chainop)
    effect::set_operation(d, e3, Some(operation(chainop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy non-Plant/Insect monsters
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `aux.FALSE`
fn aux_false(_d: &mut Duel, _a: Args) -> bool {
    false
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PLANT|RACE_INSECT) and c:IsAbleToRemoveAsCost()
    //     and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true,true))
    card::is_race(d, c, RACE_PLANT | RACE_INSECT)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, true))
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(cc) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, cc);
    // local c=e:GetHandler()
    let e = a.e(0);
    let c = effect::get_handler(d, e).nil();
    // local rg1=Duel.GetMatchingGroup(s.costfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,c)
    let rg1 = duel::get_matching_group(d, Some(&costfilter), tp, LOCATION_MZONE | LOCATION_HAND, 0, c.into());
    // local rg2=Duel.GetMatchingGroup(s.costfilter,tp,LOCATION_GRAVE|LOCATION_HAND,0,c)
    let rg2 = duel::get_matching_group(d, Some(&costfilter), tp, LOCATION_GRAVE | LOCATION_HAND, 0, c.into());
    // local rg=Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) and rg1 or rg2
    let rg = if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() { rg1 } else { rg2 };
    // return aux.SelectUnselectGroup(rg,e,tp,3,3,aux.ChkfMMZ(1),0)
    matches!(
        aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false),
        Err(true)
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local rg1=Duel.GetMatchingGroup(s.costfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,c)
    let rg1 = duel::get_matching_group(d, Some(&costfilter), tp, LOCATION_MZONE | LOCATION_HAND, 0, c.into());
    // local rg2=Duel.GetMatchingGroup(s.costfilter,tp,LOCATION_GRAVE|LOCATION_HAND,0,c)
    let rg2 = duel::get_matching_group(d, Some(&costfilter), tp, LOCATION_GRAVE | LOCATION_HAND, 0, c.into());
    // local rg=Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) and rg1 or rg2
    let rg = if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() { rg1 } else { rg2 };
    // local g=aux.SelectUnselectGroup(rg,e,tp,3,3,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(
        d,
        &rg,
        Some(e),
        tp,
        Some(3),
        Some(3),
        Some(&aux::chkf_mmz(1)),
        1,
        tp,
        Some(HINTMSG_REMOVE as u64),
        None,
        None,
        true,
    )
    .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, a.e(0)).as_group().cloned() else { return };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
/// `s.chainop(e,tp,eg,ep,ev,re,r,rp)`
fn chainop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    // if re:IsSpellTrapEffect() and re:GetOwnerPlayer()==tp then
    if effect::is_spell_trap_effect(d, re) && effect::get_owner_player(d, re) == a.tp {
        // Duel.SetChainLimit(s.chainlm)
        duel::set_chain_limit(d, lua_fn(chainlm));
    }
}
/// `s.chainlm(e,ep,tp)`
fn chainlm(d: &mut Duel, a: Args) -> bool {
    // return ep==tp or not e:IsMonsterEffect()
    a.p(1) == a.p(2) || !effect::is_monster_effect(d, a.e(0))
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not c:IsRace(RACE_PLANT|RACE_INSECT)
    card::is_faceup(d, c) && !card::is_race(d, c, RACE_PLANT | RACE_INSECT)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3207)
        effect::set_description(d, e1, 3207);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
